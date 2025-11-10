#pragma once

#include <condition_variable>
#include <deque>
#include <functional>
#include <mutex>
#include <string>
#include <utility>

// A simple task scheduler that executes move-only tasks.
class Scheduler {
public:

  // Shared state protected by mutex
  struct SharedState {
    bool is_stopped;
    std::deque<std::move_only_function<void()>> immediate_tasks;
  };

  std::string name;
  std::mutex mutex;
  std::condition_variable cv;
  SharedState state;

  Scheduler(std::string name) : name(std::move(name)) {}
  ~Scheduler() = default;
  // Non-copyable, moveable by default (rule of zero is sufficient)
  // Copy/move operators are implicitly deleted if needed

  // Add a task to the scheduler
  void schedule(std::move_only_function<void()> task) {
    std::lock_guard<std::mutex> lg(mutex);
    state.immediate_tasks.push_back(std::move(task));
  }

  // Main loop: Executes tasks until the stop() method is called
  void start_loop() {

    while (true) {
      bool is_stopped;

      {
        std::lock_guard<std::mutex> lg(mutex);
        is_stopped = state.is_stopped;
      }

      if (is_stopped) {
        break;
      }

      bool has_immediate_task;
      std::move_only_function<void()> immediate_task;

      {
        std::lock_guard<std::mutex> lg(mutex);
        has_immediate_task = !state.immediate_tasks.empty();
        if (has_immediate_task) {
          immediate_task = std::move(state.immediate_tasks.front());
          state.immediate_tasks.pop_front();
        }
      }

      if (has_immediate_task) {
        immediate_task();
      }
    }
  }

  // Stop the scheduler loop
  void stop() {
    std::lock_guard<std::mutex> lg(mutex);
    state.is_stopped = true;
  }
};
