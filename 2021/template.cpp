#include <bits/stdc++.h>
using namespace std;

vector<string> split(const string& str, const char& delimiter) {
  istringstream in(str);
  string token;
  vector<string> t;
  while(getline(in, token, delimiter)) {
    t.push_back(token);
  }
  return t;
}


int main() {

  return 0;
}

