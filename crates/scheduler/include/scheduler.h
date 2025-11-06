#pragma once

#include <condition_variable>
#include <deque>
#include <functional>
#include <mutex>
#include <string>
#include <utility>

class Scheduler {
public:
  struct SharedState {
    bool is_stopped;
    std::deque<std::function<void()>> immediate_tasks;
  };

  std::string name;
  std::mutex mutex;
  std::condition_variable cv;
  SharedState state;

  Scheduler(std::string name) : name(std::move(name)) {}
  ~Scheduler() = default;
  // Scheduler(const Scheduler&) = default;
  // Scheduler& operator=(const Scheduler&) = default;
  // Scheduler(Scheduler&&) = default;
  // Scheduler& operator=(Scheduler&&) = default;

  void schedule(std::function<void()> task) {
    std::lock_guard<std::mutex> lg(mutex);
    state.immediate_tasks.push_back(task);
  }

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
      std::function<void()> immediate_task;

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

  void stop() {
    std::lock_guard<std::mutex> lg(mutex);
    state.is_stopped = true;
  }
};
