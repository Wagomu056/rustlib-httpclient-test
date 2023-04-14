#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Post {
  unsigned int id;
  unsigned int user_id;
  const char *title;
  const char *body;
};

using RequestCallback = void(*)(bool, const Post*);

struct RequestPost {
  unsigned int user_id;
  const char *title;
  const char *body;
};

extern "C" {

void get_request(RequestCallback callback);

void post_request(const RequestPost *param, RequestCallback callback);

} // extern "C"
