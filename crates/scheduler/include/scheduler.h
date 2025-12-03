#pragma once

#include <condition_variable>
#include <deque>
#include <queue>
#include <functional>
#include <mutex>
#include <string>
#include <utility>
#include <chrono>

using DateTime = std::chrono::time_point<std::chrono::system_clock>;

class DelayedTask {
public:
  std::move_only_function<void()> task;
  DateTime duetime;

  DelayedTask(std::move_only_function<void()> task, DateTime duetime) : task(std::move(task)), duetime(duetime) {};

  bool operator<(const DelayedTask & other) const {
    return duetime > other.duetime;
  }
};

// A simple task scheduler that executes move-only tasks.
class Scheduler {
public:

  // Shared state protected by mutex
  struct SharedState {
    bool is_stopped;
    std::deque<std::move_only_function<void()>> immediate_tasks;
    std::priority_queue<DelayedTask> delayed_tasks;
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
    cv.notify_one();
  }

  void schedule_absolute(std::move_only_function<void()> task, DateTime duetime) {
    std::lock_guard<std::mutex> lg(mutex);
    DelayedTask&& delayed_task{std::move(task), duetime};
    state.delayed_tasks.push(std::move(delayed_task));
    cv.notify_one();
  }

  template <typename Rep, typename Period>
  void schedule_relative(std::move_only_function<void()> task, std::chrono::duration<Rep, Period> duration) {
    DateTime duetime = std::chrono::system_clock::now() + duration;
    schedule_absolute(std::move(task), duetime);
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
      } else {

        bool has_delayed_task;
        DateTime duetime;
        {
          std::lock_guard<std::mutex> lg(mutex);
          has_delayed_task = !state.delayed_tasks.empty();
          if (has_delayed_task) {
            auto& delayed_task = state.delayed_tasks.top();
            duetime = delayed_task.duetime;
          }
        }

        bool is_delayed_task_due;
        std::move_only_function<void()> due_task;
        // DateTime duetime;
        if (has_delayed_task) {
            if (duetime < std::chrono::system_clock::now()) {
              is_delayed_task_due = true;
              auto delayed_task = std::move(const_cast<DelayedTask&>(state.delayed_tasks.top()));
              due_task = std::move(delayed_task.task);
            }
            else {
              is_delayed_task_due = false;
            }
        }

        {
          std::unique_lock<std::mutex> lg(mutex);

          if (is_delayed_task_due) {
            state.immediate_tasks.push_back(std::move(due_task));
            state.delayed_tasks.pop();
          } else {
            if (state.immediate_tasks.empty()) {
              if (has_delayed_task) {
                auto duration = duetime - std::chrono::system_clock::now();
                if (duration > std::chrono::seconds(0)) {
                  cv.wait_for(lg, duration);
                } 
              } else {
                cv.wait(lg);
              }
            }
          }
        }

      }
    }
  }

  // Stop the scheduler loop
  void stop() {
    std::lock_guard<std::mutex> lg(mutex);
    state.is_stopped = true;
  }
};
