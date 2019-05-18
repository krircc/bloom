<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>

      <v-flex xs12 class="title blue-grey--text text--darken-1">
                <h1>
                  How to contribute to Bloom
                </h1>
              </v-flex>


               <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Introduction
                </h2>
                <p>
                  Bloom believes in a world where everyone can contribute. Our mission is to change
                  all creative work from read-only to read-write. When everyone can contribute,
                  users become contributors and we greatly increase the rate of human progress.
                </p>

                <p>
                  We want to make it as easy as possible for Bloom users to become Bloom contributors,
                  so we’ve created this guide to help you get started. We have multiple tracks to cater
                  to people of varying experience levels. <br/><br/>

                  If you’re uncomfortable getting into open source development right away, you may
                  want to consider the Documentation or Translation tracks. Documentation and
                  Translation are both just as important as code, and we'd be happy to have your
                  contributions. <br/><br/>
                </p>
              </v-flex>

              <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Overview
                </h2>
                <p>
                  BLAH BLAH BLAH: expliquer le workflow, forum, a quoi servent les issues...
                </p>
              </v-flex>


              <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Development
                </h2>
                <p>
                  These instructions are for development of any of Bloom project.<br/><br/>
                  <ul>
                    <li>Choose an issue to work on. Be sure to comment and verify no one else is
                      working on the issue, and to make sure we’re still interested in a given
                      contribution.<br/>
                      You may also want to comment and ask for help if you’re new or if you get stuck.
                      We’re more than happy to help!</li>
                    <li>Add the feature or fix the bug you’ve chosen to work on.</li>
                    <li>If it's a feature change that impacts users or admins, update the documentation.</li>
                    <li>Open a pull request to merge your code and its documentation. The earlier you
                      open a merge request, the sooner you can get feedback. You can mark it as a
                      Work in Progress to signal that you’re not done yet.</li>
                    <li>Add tests if needed, as well as a changelog entry.</li>
                    <li>Make sure the test suite is passing.</li>
                    <li>Wait for a reviewer. You’ll likely need to change some things once the reviewer
                      has completed a code review for your merge request. You may also need
                      multiple reviews depending on the size of the change.</li>
                    <li>Get your changes merged!</li>
                  </ul>
                </p>

                <h3 class="text-xs-left blue-grey--text text--darken-1 mt-5 mb-3">
                  Security notice
                </h3>
                <p>
                  For security reasons please do not include any external dependency in a Merge request.<br/>
                  Please contact a member of the core team if for some reason you think you need to incldue an
                  external dependency.
                </p>
              </v-flex>


              <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  UX Design
                </h2>
                <p>
                  These instructions are for those wanting to contribute UX designs specifically.
                  The UX department at Bloom uses Sketch for all of its designs. See the <a href="https://github.com/bloom42/design">
                  DesignRepository documentation</a> for details on working with our files.
                  Visit our Contributing guidelines to read our general guidelines for contributing.
                  While they are code-focused instructions, they will help you understand the overall process of contributing.<br/><br/>

                  You can refer to the Development worflow to propose new features / fix bugs.
                </p>
              </v-flex>


              <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Documentation
                </h2>
                <p>
                  This section pertains to documentation changes that are independent of other
                  code/feature changes. For documentation changes that accompany changes to
                  code/features, see the Development section above.<br/><br/>

                  Bloom's documentation is splitted in 2 parts:
                  <ul>
                    <li>Documentation for users: <a href="https://help.bloom.sh">https://help.bloom.sh</a>,
                    you can find the repo here: <a href="https://github.com/bloom42/help">https://github.com/bloom42/help</a>
                    You can open an issue with a proposal or open an pull request.</li>
                    <li>Documentation for developers: <a href="https://opensource.bloom.sh">https://opensource.bloom.sh</a>,
                    you can find the repo here: <a href="https://github.com/bloom42/opensource">https://github.com/bloom42/opensource</a>.
                    Each project documentation can be found within the project's repository in the <b>docs</b> folder.</li>
                  </ul>
                </p>
              </v-flex>

              <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Sponsors
                </h2>
                <p>
                  Another way to contribute to the Bloom project is financially: here is our
                  <a href="https://bloom.sh/sponsors">sponsors page</a> where you can donate.
                </p>
              </v-flex>


              <v-flex xs12 class="mb-3 text-xs-left">
                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Getting Help
                </h2>
                <p>
                  If you need help while contributing to Bloom, below are some of the resources that are available.
                  <ul>
                    <li>Ask questions on <a href="https://discordapp.com/invite/HmDQDGv">Bloom community chat</a></li>
                  </ul>
                </p>
              </v-flex>

    </v-layout>
  </v-container>

  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Contributors from '../../components/Contributors.vue';


@Component({
  components: {
    'blm-contributors': Contributors,
  },
})
export default class Index extends Vue {
  // props
  // data
  hover_elevation = 4;
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}
</style>