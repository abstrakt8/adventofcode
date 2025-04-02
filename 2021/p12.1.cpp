#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class Solver {
public:
  unordered_map<string, vector<string>> E;
  unordered_map<string, int> id;
  vector<string> small;

  int smallCave(const string &t) {
    return find(begin(small), end(small), t) - begin(small);
  }

  int dfs(string u, int mask) {
    if (u == "end") {
      return 1;
    }
    int ans = 0;
    for (auto &v : E[u]) {
      int s = smallCave(std::move(v));
      int nmask = mask;
      if (s != small.size()) {
        if ((mask >> s) & 1) {
          continue;
        } else {
          nmask |= (1 << s);
        }
      }
      ans += dfs(v, nmask);
    }
    return ans;
  }

  void solve() {
    string line;
    while (getline(cin, line)) {
      size_t pos = line.find('-');
      if (pos != string::npos) {
        string a = line.substr(0, pos);
        string b = line.substr(pos + 1);
        E[a].push_back(b);
        E[b].push_back(a);
      }
    }

    for (auto &[key, _] : E) {
      if (islower(key[0])) {
        small.push_back(key);
      }
      id[key] = id.size();
    }

    int n = E.size();
    int m = small.size();
    int initialMask = (1 << smallCave("start"));
    int ans = dfs("start", initialMask);
    cout << ans << endl;
  }
};
int main() {
  Solver solver;
  solver.solve();
}
