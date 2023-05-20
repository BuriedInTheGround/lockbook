import SwiftUI
import SwiftLockbookCore

struct FileTreeView: View {
    @EnvironmentObject var sheets: SheetState
    @EnvironmentObject var currentDoc: CurrentDocument
    @EnvironmentObject var coreService: CoreService
    @EnvironmentObject var files: FileService
    @EnvironmentObject var onboarding: OnboardingService
    @EnvironmentObject var search: SearchService
    @EnvironmentObject var sync: SyncService
    
    @State var suggestedDocBranchState: Bool = true
    @State var navigateToManageSub: Bool = false
    
    @State var searchInput: String = ""
    @State private var hideOutOfSpaceAlert = UserDefaults.standard.bool(forKey: "hideOutOfSpaceAlert")

    let currentFolder: File
    let account: Account
    
    var body: some View {
        VStack {
            SearchWrapperView(
                searchInput: $searchInput,
                mainView: mainView,
                isiOS: false)
            .searchable(text: $searchInput, placement: .navigationBarDrawer(displayMode: .automatic), prompt: "Search")
            
            HStack {
                BottomBar(onCreating: {
                    sheets.creatingInfo = CreatingInfo(parent: currentFolder, child_type: .Document)
                })
            }
        }
        
        VStack {
            if let item = currentDoc.selectedDocument {
                DocumentView(meta: item)
            } else {
                GeometryReader { geometry in
                    if geometry.size.height > geometry.size.width {
                        VStack {
                            Image(systemName: "rectangle.portrait.lefthalf.inset.filled")
                                .font(.system(size: 60))
                                .padding(.bottom, 10)
                            
                            
                            Text("No document is open. Expand the file tree by swiping from the left edge of the screen or clicking the button on the top left corner.")
                                .font(.title2)
                                .multilineTextAlignment(.center)
                                .frame(maxWidth: 350)
                        }
                        .padding(.horizontal)
                        .frame(maxWidth: .infinity, maxHeight: .infinity)
                    } else {
                        EmptyView()
                    }
                }
            }
        }
        .toolbar {
            ToolbarItemGroup(placement: .navigationBarTrailing) {
                NavigationLink(
                    destination: PendingSharesView()) {
                        Image(systemName: "person.2.fill")
                            .foregroundColor(.blue)
                    }
                    
                NavigationLink(
                    destination: SettingsView().equatable(), isActive: $onboarding.theyChoseToBackup) {
                        Image(systemName: "gearshape.fill")
                            .foregroundColor(.blue)
                            .padding(.horizontal, 10)
                    }
            }
        }
        .alert(isPresented: Binding(get: { sync.outOfSpace && !hideOutOfSpaceAlert }, set: {_ in sync.outOfSpace = false })) {
            Alert(
                title: Text("Out of Space"),
                message: Text("You have run out of space!"),
                primaryButton: .default(Text("Upgrade now"), action: {
                    navigateToManageSub = true
                }),
                secondaryButton: .default(Text("Don't show me this again"), action: {
                    hideOutOfSpaceAlert = true
                    UserDefaults.standard.set(hideOutOfSpaceAlert, forKey: "hideOutOfSpaceAlert")
                })
            )
        }
        .background(
            NavigationLink(destination: ManageSubscription(), isActive: $navigateToManageSub, label: {
                EmptyView()
            })
            .hidden()
        )
        .onChange(of: currentDoc.selectedDocument) { _ in
            DI.files.refreshSuggestedDocs()
        }
    }
    
    var mainView: some View {
        VStack(alignment: .leading) {
            suggestedDocs
            
            Text("Files")
                .bold()
                .foregroundColor(.primary)
                .textCase(.none)
                .font(.headline)
                .padding(.top)
                .padding(.bottom, 5)
            
            OutlineSection(root: currentFolder)
        }
        .padding(.horizontal)
    }
    
    var suggestedDocs: some View {
        Group {
            Button(action: {
                withAnimation {
                    suggestedDocBranchState.toggle()
                }
            }) {
                HStack {
                    Text("Suggested")
                        .bold()
                        .foregroundColor(.primary)
                        .textCase(.none)
                        .font(.headline)
                    
                    Spacer()
                    
                    if suggestedDocBranchState {
                        Image(systemName: "chevron.down")
                            .foregroundColor(.gray)
                            .imageScale(.small)
                    } else {
                        Image(systemName: "chevron.right")
                            .foregroundColor(.gray)
                            .imageScale(.small)
                    }
                }
                .padding(.top)
                .padding(.bottom, 5)
                .contentShape(Rectangle())
            }
            
            if suggestedDocBranchState {
                SuggestedDocs(isiOS: false)
                Spacer()
            } else {
                Spacer()
            }
        }
    }
}
