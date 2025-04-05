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

using Points = set<array<int, 2>>;

Points foldVertical(const Points& p, int X) {
  Points q;
  for(auto& [x, y] : p) {
    if(x < X) {
      q.insert({x, y});
    } else {
      q.insert({2*X - x, y});
    }
  }
  return q;
}

Points foldHorizontal(const Points& p, int Y) {
  Points q;
  for(auto& [x, y] : p) {
    if(y < Y) {
      q.insert({x, y});
    } else {
      q.insert({x, 2*Y - y});
    }
  }
  return q;
}

int main() {
  string line;
  Points pts;
  while(getline(cin, line)) {
    if(line.empty()) {
      break;
    }
    vector<string> s = split(line, ',');
    int x = stoi(s[0]);
    int y = stoi(s[1]);
    pts.insert({x, y});
  }
  cout << "n = " << pts.size() << endl;
  cout << "Part 1: " << foldVertical(pts, 655).size() << endl;

  while(getline(cin, line) && !line.empty()) {
    vector<string> s = split(line, '=');
    char c = s[0].back();
    int val = stoi(s[1]);
    cout << c << " " << val << endl;
    if(c == 'x') {
      pts = foldVertical(pts, val);
    } else {
      pts = foldHorizontal(pts, val);
    }
  }

  cout << "Part 2: " << pts.size() << endl;
  int max_x = 0, max_y = 0;
  for(auto & [x, y] : pts ) {
    max_x = max(x, max_x);
    max_y = max(y, max_y);
  }
  for(int y = 0; y <= max_y; y++) {
    for(int x = 0; x <= max_x; x++) {
      cout << (pts.count({x, y}) ? '#' : '.');
    }
    cout << endl;
  }


  return 0;
}

