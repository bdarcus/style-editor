import React from 'react';
import { MainLayout } from '../layouts/MainLayout';
import { Link } from 'react-router-dom';

export const HomePage: React.FC = () => {
    return (
        <MainLayout>
            <div className="flex flex-col gap-8">
                <div className="text-center py-16">
                    <h1 className="text-4xl font-black text-text-main mb-4 tracking-tight">The Next Generation of CSL Citation Styles</h1>
                    <p className="text-text-secondary text-lg max-w-2xl mx-auto">
                        Find, edit, and create CSL citation styles for your research.
                    </p>

                    <div className="mt-10 max-w-2xl mx-auto relative group">
                        <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                            <span className="material-symbols-outlined text-text-secondary text-2xl group-focus-within:text-primary transition-colors">search</span>
                        </div>
                        <input
                            type="text"
                            placeholder="Search by name, publisher, or field (e.g. 'APA', 'Nature', 'Biology')..."
                            className="block w-full rounded-2xl border border-border-light bg-white py-4 pl-12 pr-4 text-lg shadow-xl shadow-blue-500/5 placeholder:text-text-secondary/60 focus:ring-2 focus:ring-primary focus:border-transparent transition-all outline-none"
                        />
                    </div>

                    <div className="mt-8 flex justify-center gap-4">
                        <Link to="/create-wizard" className="bg-primary text-white px-6 py-3 rounded-xl font-bold hover:bg-blue-700 shadow-lg shadow-blue-500/20 transition-all">
                            Create New Style
                        </Link>
                        <button className="bg-white border border-border-light text-text-main px-6 py-3 rounded-xl font-bold hover:bg-gray-50 transition-colors shadow-sm">
                            Browse Repository
                        </button>
                    </div>
                </div>

                {/* Placeholder for list of styles */}
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div className="bg-white p-6 rounded-xl border border-border-light shadow-sm hover:shadow-md transition-shadow group">
                        <div className="size-10 bg-blue-50 text-primary rounded-lg flex items-center justify-center mb-4 group-hover:bg-primary group-hover:text-white transition-colors">
                            <span className="material-symbols-outlined">description</span>
                        </div>
                        <h3 className="text-xl font-bold text-text-main mb-2">APA 7th Edition</h3>
                        <p className="text-text-secondary text-sm mb-4">American Psychological Association</p>
                        <div className="flex gap-2">
                            <Link to="/style/apa-7" className="text-primary text-sm font-bold hover:underline">View Details</Link>
                        </div>
                    </div>

                    <div className="bg-white p-6 rounded-xl border border-border-light shadow-sm hover:shadow-md transition-shadow group">
                        <div className="size-10 bg-blue-50 text-primary rounded-lg flex items-center justify-center mb-4 group-hover:bg-primary group-hover:text-white transition-colors">
                            <span className="material-symbols-outlined">science</span>
                        </div>
                        <h3 className="text-xl font-bold text-text-main mb-2">Nature</h3>
                        <p className="text-text-secondary text-sm mb-4">Nature Publishing Group</p>
                        <div className="flex gap-2">
                            <Link to="/style/nature" className="text-primary text-sm font-bold hover:underline">View Details</Link>
                        </div>
                    </div>

                    <div className="bg-white p-6 rounded-xl border border-border-light shadow-sm hover:shadow-md transition-shadow group">
                        <div className="size-10 bg-blue-50 text-primary rounded-lg flex items-center justify-center mb-4 group-hover:bg-primary group-hover:text-white transition-colors">
                            <span className="material-symbols-outlined">history_edu</span>
                        </div>
                        <h3 className="text-xl font-bold text-text-main mb-2">Chicago 17th (Author-Date)</h3>
                        <p className="text-text-secondary text-sm mb-4">University of Chicago Press</p>
                        <div className="flex gap-2">
                            <Link to="/style/chicago-author-date" className="text-primary text-sm font-bold hover:underline">View Details</Link>
                        </div>
                    </div>
                </div>
            </div>
        </MainLayout>
    );
};
