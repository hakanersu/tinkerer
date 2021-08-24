import Vue from 'vue'
import App from './App.vue'
import * as VueMenu from '@hscmap/vue-menu'
Vue.use(VueMenu)
Vue.config.productionTip = false

new Vue({
  render: h => h(App),
}).$mount('#app')
