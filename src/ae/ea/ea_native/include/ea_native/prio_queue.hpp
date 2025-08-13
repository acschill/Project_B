
#pragma once
#include <queue>
namespace ea_native {
struct Job{int lane; unsigned long long deadline_ns; unsigned long long id;};
struct DeadlineCmp{bool operator()(const Job&a,const Job&b)const{return a.deadline_ns>b.deadline_ns;}};
class PriQueue{public: void schedule(const Job& j){qs_[j.lane].push(j);} bool poll_ready(int lane, Job& out){auto& q=qs_[lane]; if(q.empty()) return false; out=q.top(); q.pop(); return true;} private: std::priority_queue<Job,std::vector<Job>,DeadlineCmp> qs_[3];};
}
