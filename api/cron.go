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

func InitCronner() Cron {
	return Cron{
		ptr: C.init_cronner(),
	}
}

func StartCronJob(cron Cron) {
	C.start_cron_job(cron.ptr)
}
