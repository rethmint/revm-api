package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

type Cron struct {
	ptr *C.cron_t
}

func ReleaseCron(cron Cron) {
	C.release_cron(cron.ptr)
}

func InitCron() Cron {
	return Cron{
		ptr: C.init_cron_job(),
	}
}

func JoinCron(cron Cron) {
	C.join_cron(cron.ptr)
}
