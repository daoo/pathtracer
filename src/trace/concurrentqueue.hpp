#ifndef CONCURRENTQUEUE_HPP_CEBSEVZN
#define CONCURRENTQUEUE_HPP_CEBSEVZN

#include <condition_variable>
#include <mutex>
#include <queue>

namespace misc
{
  template<typename T>
  class ConcurrentQueue
  {
    public:
      void push(const T& data)
      {
        std::unique_lock<std::mutex> lock(m_mutex);
        m_queue.push(data);
        lock.unlock();
        m_cond.notify_one();
      }

      bool empty() const
      {
        std::lock_guard<std::mutex> lock(m_mutex);
        return m_queue.empty();
      }

      bool try_pop(T& val)
      {
        std::lock_guard<std::mutex> lock(m_mutex);

        if (m_queue.empty())
          return false;

        val = m_queue.front();
        m_queue.pop();
        return true;
      }

      void wait_and_pop(T& val)
      {
        std::unique_lock<std::mutex> lock(m_mutex);

        while (m_queue.empty())
          m_cond.wait(lock);

        val = m_queue.front();
        m_queue.pop();
      }

    private:
      std::queue<T> m_queue;
      std::mutex m_mutex;
      std::condition_variable m_cond;
  };
}

#endif /* end of include guard: CONCURRENTQUEUE_HPP_CEBSEVZN */
