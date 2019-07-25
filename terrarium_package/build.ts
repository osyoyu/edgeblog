import * as gulp from "gulp";
import { Service, project } from "@wasm/studio-utils";

gulp.task("deploy", async () => {
  await Service.deployFiles([project.getFile("module.wasm"), ...project.globFiles("assets/**")], "wasm");
});

gulp.task("default", ["deploy"]);
