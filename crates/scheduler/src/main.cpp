#include <print>

#include "scheduler.h"

int main() {
  Scheduler scheduler("s1");

  scheduler.schedule([&scheduler](){
    std::println("Hello from s1");
    scheduler.stop();
  });

  scheduler.start_loop();

  std::println("Hello, World!");
}
