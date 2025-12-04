#include <print>

#include "scheduler.h"

std::move_only_function<void()> count_down(Scheduler& scheduler, int counter) {
  return [&scheduler, counter](){

    std::println("{}:{}", scheduler.name, counter);
    if (counter > 0) {
      auto && task = count_down(scheduler, counter - 1);
      scheduler.schedule_relative(std::move(task), std::chrono::seconds(1));
    } else {
      scheduler.stop();
    }
  };
}

int main() {
  Scheduler scheduler("s1");

  auto && task = count_down(scheduler, 5);
  scheduler.schedule(std::move(task));
  scheduler.start_loop();
}
