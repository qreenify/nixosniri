// Example: GTK4 window without traditional headerbar in C

#include <gtk/gtk.h>
#include <adwaita.h>

static void on_close_clicked(GtkWidget *widget, gpointer data) {
    GtkWindow *window = GTK_WINDOW(data);
    gtk_window_destroy(window);
}

static void activate(GtkApplication *app, gpointer user_data) {
    GtkWidget *window;
    GtkWidget *main_box;
    GtkWidget *control_bar;
    GtkWidget *close_btn;
    GtkWidget *title_label;
    GtkWidget *content;

    // Create window without decorations
    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Settings");
    gtk_window_set_default_size(GTK_WINDOW(window), 800, 600);

    // Remove window decorations (title bar, borders)
    gtk_window_set_decorated(GTK_WINDOW(window), FALSE);

    // Create main vertical box
    main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_window_set_child(GTK_WINDOW(window), main_box);

    // Create custom control bar (since we removed decorations)
    control_bar = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 6);
    gtk_widget_set_margin_top(control_bar, 6);
    gtk_widget_set_margin_start(control_bar, 12);
    gtk_widget_set_margin_end(control_bar, 12);
    gtk_box_append(GTK_BOX(main_box), control_bar);

    // Add title label
    title_label = gtk_label_new("Settings");
    gtk_widget_set_hexpand(title_label, TRUE);
    gtk_widget_set_halign(title_label, GTK_ALIGN_START);
    gtk_box_append(GTK_BOX(control_bar), title_label);

    // Add close button
    close_btn = gtk_button_new_with_label("×");
    g_signal_connect(close_btn, "clicked", G_CALLBACK(on_close_clicked), window);
    gtk_box_append(GTK_BOX(control_bar), close_btn);

    // Add main content area
    content = gtk_label_new("Main Settings Content Area");
    gtk_widget_set_vexpand(content, TRUE);
    gtk_box_append(GTK_BOX(main_box), content);

    // Show window
    gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
    AdwApplication *app;
    int status;

    // Initialize Adwaita
    app = adw_application_new("com.example.NoHeaderbarDemo", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);

    status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}

// Compile with:
// gcc -o demo no_headerbar_demo.c `pkg-config --cflags --libs gtk4 libadwaita-1`