#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class Solver {
public:
  unordered_map<string, vector<string>> E;
  unordered_map<string, int> vis;
  int smallTwice = 0;

  int dfs(const string &u) {
    if (u == "end") {
      return 1;
    }
    int ans = 0;
    for (auto &v : E[u]) {
      if (v == "start")
        continue;
      auto it = vis.find(v);
      if (it != vis.end()) {
        if (it->second == 1) {
          if (smallTwice) {
            continue;
          } else {
            smallTwice += 1;
            it->second += 1;
          }
        } else if (it->second == 0) {
          it->second += 1;
        } else {
          continue;
        }
      }
      ans += dfs(v);
      if (it != vis.end()) {
        smallTwice -= it->second-- == 2;
      }
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
        vis[key] = 0;
      }
    }

    int n = E.size();
    int ans = dfs("start");
    cout << ans << endl;
  }
};
int main() {
  Solver solver;
  solver.solve();
}
