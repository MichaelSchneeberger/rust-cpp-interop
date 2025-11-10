#pragma once

#include <functional>
#include <memory>
#include "scheduler.h"
#include "scheduler/src/lib.rs.h"

struct DynFunOnce;

std::shared_ptr<Scheduler> new_scheduler() {
  return std::make_shared<Scheduler>("s1");
}

void schedule(std::shared_ptr<Scheduler> s, rust::Box<DynFunOnce> t) {

  // The closure is defined mutable, because std::move(t) modifies
  // the captured value t.
  s->schedule([t = std::move(t)]() mutable {
    execute_dyn_fun_once(std::move(t));
  });

}

// void schedule(std::shared_ptr<Scheduler> s, rust::Fn<void(std::shared_ptr<Scheduler>)> t) {
//   s->schedule([t, s](){t(s);});
// }

// void schedule_with_ctx(
//   std::shared_ptr<Scheduler> s, 
//   rust::Fn<void(std::shared_ptr<Scheduler>)> t, 
//   std::shared_ptr<Scheduler> ctx
// ) {
//   s->schedule([t, ctx](){t(ctx);});
// }

void start_loop(std::shared_ptr<Scheduler> s) {
  s->start_loop();
}

void stop(std::shared_ptr<Scheduler> s) {
  s->stop();
}

