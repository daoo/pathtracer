#ifndef UTIL_CONCURRENTQUEUE_H_
#define UTIL_CONCURRENTQUEUE_H_

#include <condition_variable>
#include <mutex>
#include <queue>

namespace util {
template <typename T>
class ConcurrentQueue {
 public:
  void push(const T& data) {
    std::unique_lock<std::mutex> lock(mutex_);
    queue_.push(data);
    lock.unlock();
    cond_.notify_one();
  }

  bool empty() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return queue_.empty();
  }

  bool try_pop(T& val) {
    std::lock_guard<std::mutex> lock(mutex_);

    if (queue_.empty()) return false;

    val = queue_.front();
    queue_.pop();
    return true;
  }

  void wait_and_pop(T& val) {
    std::unique_lock<std::mutex> lock(mutex_);

    while (queue_.empty())
      cond_.wait(lock);

    val = queue_.front();
    queue_.pop();
  }

 private:
  std::queue<T> queue_;
  std::mutex mutex_;
  std::condition_variable cond_;
};
}  // namespace util

#endif  // UTIL_CONCURRENTQUEUE_H_
