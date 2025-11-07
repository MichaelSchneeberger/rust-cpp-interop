#pragma once

#include <functional>
#include <memory>
#include "scheduler.h"
#include "scheduler/src/lib.rs.h"

std::shared_ptr<Scheduler> new_scheduler() {
  return std::make_shared<Scheduler>("s1");
}

void schedule(std::shared_ptr<Scheduler> s, rust::Fn<void(std::shared_ptr<Scheduler>)> t) {
  s->schedule([t, s](){t(s);});
}

void schedule_with_ctx(std::shared_ptr<Scheduler> s, rust::Fn<void(std::shared_ptr<Scheduler>)> t, std::shared_ptr<Scheduler> ctx) {
  s->schedule([t, ctx](){t(ctx);});
}

void start_loop(std::shared_ptr<Scheduler> s) {
  s->start_loop();
}

void stop(std::shared_ptr<Scheduler> s) {
  s->stop();
}

