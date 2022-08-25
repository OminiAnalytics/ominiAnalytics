/*
 * @Author: realbacon
 * @Date: 2022-08-25 11:32:37
 * @Last Modified by:   realbacon
 * @Last Modified time: 2022-08-25 11:32:37
 */

class OminiTracker {
    constructor(key) {
        this.api = new ApiAsync("http://127.0.0.1:5000/", key);
        this.user = new User(this.api);
    }

    initTracker() {
        this.user.init();
    }
}

/*
<script src="https://cdn/omini-tracker.min.js" async defer>
const ominitracker = new OminiTracker("123456789");
ominitracker.initTracker();
</script>
*/
