<template>
  <div>
    <v-list>

      <v-list-tile exact to="/drive">
        <v-list-tile-action>
          <v-icon>mdi-desktop-tower</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>My Drive</v-list-tile-title>
        </v-list-tile-content>
      </v-list-tile>

      <v-list-tile exact to="/drive/trash">
        <v-list-tile-action>
          <v-icon>mdi-delete</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>Trash</v-list-tile-title>
        </v-list-tile-content>
      </v-list-tile>

      <v-divider></v-divider>
    </v-list>


    <v-list three-line v-if="$store.state.drive_profile" class="pointer">
      <v-list-tile @click="open_add_payment_dialog">
        <v-list-tile-action>
          <v-icon>mdi-cloud-outline</v-icon>
        </v-list-tile-action>
        <v-list-tile-content>
          <v-list-tile-title>Storage</v-list-tile-title>
          <v-list-tile-sub-title>
            <v-progress-linear  :value="used_percent"></v-progress-linear>
          </v-list-tile-sub-title>
          <v-list-tile-sub-title>
            {{ $store.state.drive_profile.used_space | filesize }}
            used of {{ $store.state.drive_profile.total_space | filesize }}
          </v-list-tile-sub-title>
        </v-list-tile-content>
      </v-list-tile>
    </v-list>

    <blm-drive-dialog-add-payment
      :visible="add_payment_dialog"
      @close="close_add_payment_dialog" />
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import AddPaymentDialog from './AddPaymentDialog.vue';


@Component({
  components: {
    'blm-drive-dialog-add-payment': AddPaymentDialog,
  },
})
export default class Drawer extends Vue {
  // props
  // data
  add_payment_dialog = false;


  // computed
  get used_percent() {
    const profile = this.$store.state.drive_profile;
    if (profile) {
      return profile.used_space / profile.total_space * 100;
    }
    return 0;
  }


  // lifecycle
  // watch
  // methods
  open_add_payment_dialog() {
    this.add_payment_dialog = true;
  }

  close_add_payment_dialog() {
    this.add_payment_dialog = false;
  }
}
</script>


<style scoped lang="scss">
/* fix storage display */
.v-list__tile__sub-title, .v-list__tile__title, .v-list__tile__content {
  overflow: visible;
}
</style>
