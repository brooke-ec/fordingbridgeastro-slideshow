import { SUPABASE_URL, SUPABASE_KEY } from "../config";
import { createClient } from "@supabase/supabase-js";
import type { Database } from "./types";

export const supabase = createClient<Database>(SUPABASE_URL, SUPABASE_KEY);
