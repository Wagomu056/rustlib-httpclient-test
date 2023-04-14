//
//  ViewController.m
//  test_app
//
//  Created by 東口拓也 on 2023/03/11.
//

#import "ViewController.h"
#import "httpclient.h"

@interface ViewController ()

@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    // Do any additional setup after loading the view.
    
    NSLog(@"prev call request");
    get_request( [](bool is_success, const Post* post){
        NSLog(@"request success: %d", is_success ? 1 : 0);
        NSLog(@"title: %s", post->title);
    } );
    NSLog(@"post call request");
}


@end
