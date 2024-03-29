<template>
  <div>
    <v-container fluid grid-list-lg>
      <v-layout align-center justify-center>
        <v-flex xs12 sm6>
          <v-text-field placeholder="Take a note..." solo @click="open_note_dialog" readonly/>
        </v-flex>
      </v-layout>
      <v-layout row wrap justify-left class="mt-1">
        <v-flex v-for="note in notes" :key="note.id" xs12 sm4 md3>
          <blm-notes-note
            :note="note"
            @archive="note_archived"
            @remove="note_removed"
            @update="note_updated"
            @delete="note_deleted"
          />
        </v-flex>
    </v-layout>
    </v-container>

  <blm-notes-dialog-note
    :visible="note_dialog"
    @close="note_dialog_closed"
    @create="note_created"
    @update="note_updated"
  />
  </div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import Note from './Note.vue';
import NoteDialog from './NoteDialog.vue';


@Component({
  components: {
    'blm-notes-dialog-note': NoteDialog,
    'blm-notes-note': Note,
  },
})
export default class Notes extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  notes: any[] = [];
  note_dialog = false;

  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.notes = await api.get(`${api.NOTES}/v1/notes`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  open_note_dialog() {
    this.note_dialog = true;
  }

  note_dialog_closed() {
    this.note_dialog = false;
  }

  note_created(note: any) {
    this.notes = [note, ...this.notes];
  }

  note_updated(updated_note: any) {
    const pos = this.notes.map((note: any) =>  note.id).indexOf(updated_note.id);
    this.notes.splice(pos, 1);
    this.notes = [updated_note, ...this.notes];
  }

  note_archived(archived_note: any) {
    this.notes = this.notes.filter((note) => note.id !== archived_note.id);
  }

  note_removed(removed_note: any) {
    this.notes = this.notes.filter((note) => note.id !== removed_note.id);
  }

  note_deleted(deleted_note: any) {
    this.notes = this.notes.filter((note) => note.id !== deleted_note.id);
  }
}
</script>


<style scoped lang="scss">
</style>
